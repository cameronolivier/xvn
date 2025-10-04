# xvn.ps1 - PowerShell integration for xvn
# Automatically switches Node.js version when entering directories with version files

#Requires -Version 5.1

# Suppress PSScriptAnalyzer warnings for intentional design choices
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSAvoidGlobalVars', '', Justification = 'Global state required for prompt integration')]
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSAvoidUsingInvokeExpression', '', Justification = 'Required for executing dynamic shell commands from xvn binary')]
param()

# Store the last directory to detect changes
$global:XVN_LAST_DIR = $PWD.Path
$global:XVN_ACTIVE_KEY = $null

# Store original prompt function
if (-not (Test-Path function:global:__xvn_original_prompt)) {
    $global:__xvn_original_prompt = $function:prompt
}

# Find version file by walking up directory tree
function Find-VersionFile {
    param([string]$StartPath)

    $current = Get-Item $StartPath
    $patterns = @('.nvmrc', '.node-version', 'package.json')

    while ($current) {
        foreach ($pattern in $patterns) {
            $file = Join-Path $current.FullName $pattern
            if (Test-Path $file) {
                if ($env:XVN_DEBUG) {
                    Write-Host "xvn: Found version file: $file" -ForegroundColor Yellow
                }
                return $file
            }
        }

        $current = $current.Parent
    }

    return $null
}

# Compute hash of file content for idempotency
function Get-ContentHash {
    param([string]$FilePath)

    if (-not (Test-Path $FilePath)) {
        return $null
    }

    $hash = Get-FileHash -Path $FilePath -Algorithm MD5
    return $hash.Hash
}

# Execute activation commands from xvn binary
function Invoke-XvnActivation {
    param([string]$Directory)

    try {
        # Call xvn activate with current directory
        $output = & xvn activate $Directory 2>&1

        if ($env:XVN_DEBUG) {
            Write-Host "xvn: Raw output from binary:" -ForegroundColor Cyan
            Write-Host $output -ForegroundColor Gray
        }

        # Parse JSON command protocol
        if ($output -match '__XVN_COMMANDS_START__(.*)__XVN_COMMANDS_END__') {
            $jsonContent = $matches[1].Trim()
            $commands = ($jsonContent | ConvertFrom-Json).commands

            if ($env:XVN_DEBUG) {
                Write-Host "xvn: Executing $($commands.Count) commands" -ForegroundColor Cyan
            }

            foreach ($cmd in $commands) {
                if ($env:XVN_DEBUG) {
                    Write-Host "xvn: $cmd" -ForegroundColor Green
                }
                Invoke-Expression $cmd
            }
        }
        elseif ($output -match "error|failed") {
            # Only show errors if XVN_DEBUG is set
            if ($env:XVN_DEBUG) {
                Write-Host "xvn error: $output" -ForegroundColor Red
            }
        }
    }
    catch {
        # Silently fail to avoid breaking the shell
        if ($env:XVN_DEBUG) {
            Write-Host "xvn exception: $_" -ForegroundColor Red
        }
    }
}

# Main hook function called on directory change
function Invoke-XvnChpwd {
    $currentDir = $PWD.Path

    # Only trigger if directory changed
    if ($currentDir -ne $global:XVN_LAST_DIR) {
        $global:XVN_LAST_DIR = $currentDir

        # Find version file
        $versionFile = Find-VersionFile -StartPath $currentDir

        if ($versionFile) {
            # Compute activation key (file path + content hash)
            $contentHash = Get-ContentHash -FilePath $versionFile
            $activationKey = "$versionFile::$contentHash"

            # Only activate if key changed (idempotency)
            if ($activationKey -ne $global:XVN_ACTIVE_KEY) {
                $global:XVN_ACTIVE_KEY = $activationKey

                if ($env:XVN_DEBUG) {
                    Write-Host "xvn: Activating for $versionFile" -ForegroundColor Yellow
                }

                Invoke-XvnActivation -Directory $currentDir
            }
            elseif ($env:XVN_DEBUG) {
                Write-Host "xvn: Already active for $versionFile" -ForegroundColor Gray
            }
        }
        else {
            # No version file found, clear active key
            $global:XVN_ACTIVE_KEY = $null

            if ($env:XVN_DEBUG) {
                Write-Host "xvn: No version file found in $currentDir" -ForegroundColor Gray
            }
        }
    }
}

# Override prompt to trigger on directory change
function global:prompt {
    Invoke-XvnChpwd
    & $global:__xvn_original_prompt
}

# Initial activation for current directory
Invoke-XvnChpwd
