# Milestone 7: Interactive Setup Wizard - Tasks

## Phase 1: Foundation (Week 1)

### Task 1.1: Add inquire dependency
- [x] Add `inquire` crate to Cargo.toml
- [x] Test basic prompts work (Confirm, Select, MultiSelect)
- [x] Document version pinning decision

### Task 1.2: Create init module structure
- [x] Create `src/init/` directory
- [x] Create `src/init/mod.rs` with public API
- [x] Create placeholder files: wizard.rs, prompts.rs, detection.rs, validation.rs
- [x] Add module to `src/lib.rs`
- [x] Add basic module documentation

### Task 1.3: Add Init command to CLI
- [x] Add `Init` variant to `Commands` enum in `src/cli.rs`
- [x] Add command documentation and help text
- [x] Add `--quick`, `--force`, `--shell`, `--non-interactive` flags
- [x] Wire up command to init module
- [x] Keep `Setup` as alias pointing to `Init`

## Phase 2: Detection Logic (Week 1)

### Task 2.1: Implement shell detection
- [x] Implement `detect_shell()` in `init/detection.rs`
- [x] Parse `$SHELL` environment variable
- [x] Support bash and zsh
- [x] Return shell type and profile path
- [x] Add tests with mocked environment

### Task 2.2: Implement version manager detection
- [x] Implement `detect_version_managers()` returning Vec of detected managers
- [x] Implement `check_nvm()` - check `~/.nvm/nvm.sh`, `$NVM_DIR`
- [x] Implement `check_fnm()` - check `which fnm`, `~/.fnm`
- [x] Implement `check_n()` - check `which n`, `/usr/local/n`
- [x] Return manager name and installation path
- [x] Add tests with mocked filesystem

### Task 2.3: Implement TTY detection
- [x] Implement `is_interactive()` checking if stdin is TTY
- [x] Use `atty` crate or `std::io::IsTerminal`
- [x] Handle `--non-interactive` flag
- [x] Add tests for interactive/non-interactive modes

## Phase 3: Prompt Functions (Week 1-2)

### Task 3.1: Implement shell selection prompt
- [x] Implement `prompt_shell()` in `init/prompts.rs`
- [x] Show detected shell with confirmation (Confirm prompt)
- [x] If declined, show Select prompt with available shells
- [x] Display shell profile path
- [x] Return selected Shell type
- [x] Add tests with simulated input

### Task 3.2: Implement plugin selection prompt
- [x] Implement `prompt_plugins()` in `init/prompts.rs`
- [x] Use MultiSelect with detected managers pre-selected
- [x] Show manager name and installation path
- [x] After selection, prompt for priority order
- [x] Return Vec<String> of plugin names in priority order
- [x] Handle case where no managers detected (show warning)
- [x] Add tests with various detection scenarios

### Task 3.3: Implement auto-install mode prompt
- [x] Implement `prompt_auto_install()` in `init/prompts.rs`
- [x] Use Select with 3 options: Prompt, Always, Never
- [x] Add educational descriptions for each option
- [x] Default to "Prompt" (safest option)
- [x] Return AutoInstallMode enum
- [x] Add tests for all options

### Task 3.4: Implement version files prompt
- [x] Implement `prompt_version_files()` in `init/prompts.rs`
- [x] Use MultiSelect with .nvmrc, .node-version, .tool-versions
- [x] Pre-select .nvmrc and .node-version (defaults)
- [x] After selection, prompt for priority order
- [x] Return Vec<String> of filenames in priority order
- [x] Add tests for different selections

### Task 3.5: Implement configuration review prompt
- [x] Implement `prompt_confirm_config()` in `init/prompts.rs`
- [x] Display formatted summary of all configuration
- [x] Use colored output for readability
- [x] Show config file path and shell profile path
- [x] Use Confirm prompt for final approval
- [x] Return bool (confirmed or not)
- [x] Add tests with various configs

## Phase 4: Wizard Orchestration (Week 2)

### Task 4.1: Implement wizard state management
- [x] Define `WizardState` struct in `init/wizard.rs`
- [x] Fields: shell, plugins, auto_install, version_files
- [x] Implement `Default` for sensible defaults
- [x] Implement conversion to `Config` struct
- [x] Add tests for state transitions

### Task 4.2: Implement interactive wizard flow
- [x] Implement `run_wizard()` in `init/wizard.rs`
- [x] Step 1: Call `prompt_shell()`, update state
- [x] Step 2: Call `prompt_plugins()`, update state
- [x] Step 3: Call `prompt_auto_install()`, update state
- [x] Step 4: Call `prompt_version_files()`, update state
- [x] Step 5: Call `prompt_confirm_config()`, confirm state
- [x] Return WizardState or error
- [x] Add progress indicators (Step X/5)
- [x] Add tests for full flow

### Task 4.3: Implement quick mode
- [x] Implement `run_quick_setup()` in `init/wizard.rs`
- [x] Auto-detect shell
- [x] Auto-detect version managers
- [x] Use defaults: auto_install=prompt, version_files=[.nvmrc, .node-version]
- [x] Skip all prompts
- [x] Log decisions to stdout
- [x] Return WizardState
- [x] Add tests for quick mode

### Task 4.4: Implement non-interactive mode
- [x] Implement `run_non_interactive()` in `init/wizard.rs`
- [x] Similar to quick mode but with logging
- [x] Use CLI flags if provided (--shell, etc.)
- [x] Fall back to detection and defaults
- [x] Return WizardState or error
- [x] Add tests for non-interactive mode

## Phase 5: Configuration & Installation (Week 2)

### Task 5.1: Implement configuration validation
- [x] Implement `validate_config()` in `init/validation.rs`
- [x] Validate shell is supported (bash or zsh)
- [x] Validate at least one plugin selected
- [x] Validate auto_install mode is valid
- [x] Validate version_files not empty
- [x] Return Result with validation errors
- [x] Add tests for valid and invalid configs

### Task 5.2: Implement config file generation
- [x] Implement `generate_config()` in `init/wizard.rs`
- [x] Convert WizardState to Config struct
- [x] Add header comments with timestamp and instructions
- [x] Serialize to YAML format
- [x] Return formatted config string
- [x] Add tests for various configurations

### Task 5.3: Implement config file writing
- [x] Implement `write_config()` in `init/wizard.rs`
- [x] Check if config exists, handle --force flag
- [x] If exists and not force, ask to overwrite
- [x] Preserve custom comments if modifying existing config
- [x] Write to ~/.xvnrc with proper permissions
- [x] Return path to written config
- [x] Add tests for new and existing configs

### Task 5.4: Integrate with existing SetupInstaller
- [x] Reuse `setup::SetupInstaller` for shell integration
- [x] Call `installer.install()` after config written
- [x] Pass shell type from wizard state
- [x] Capture installation result
- [x] Add tests for integration

## Phase 6: User Feedback & Output (Week 2)

### Task 6.1: Implement wizard header and branding
- [x] Implement `print_wizard_header()` in `init/wizard.rs`
- [x] Use `output::print_header()` for branding
- [x] Add welcome message
- [x] Explain what wizard will do
- [x] Add visual separator

### Task 6.2: Implement success message
- [x] Implement `print_success_message()` in `init/wizard.rs`
- [x] Use colored output module
- [x] Show config file location
- [x] Show shell profile modified
- [x] Show next steps (restart shell, test)
- [x] Mention re-running wizard to modify

### Task 6.3: Implement educational help text
- [x] Add help text for each wizard step
- [x] Explain what each option does
- [x] Provide examples where helpful
- [x] Add links to documentation
- [x] Keep text concise but informative

### Task 6.4: Implement error messages
- [x] Implement error handling for each step
- [x] Use `output::error()` for errors
- [x] Provide actionable next steps
- [x] Handle user cancellation gracefully (Ctrl+C)
- [x] Log errors appropriately

## Phase 7: Testing & Documentation (Week 3)

### Task 7.1: Write unit tests
- [ ] Test all detection functions with mocked data
- [ ] Test all prompt functions with simulated input
- [ ] Test validation functions with valid/invalid configs
- [ ] Test state management and transitions
- [ ] Achieve >85% code coverage

### Task 7.2: Write integration tests
- [ ] Test full wizard flow end-to-end
- [ ] Test quick mode flow
- [ ] Test non-interactive mode
- [ ] Test re-running wizard with existing config
- [ ] Test with various version manager combinations

### Task 7.3: Manual testing
- [ ] Test on clean system (no config)
- [ ] Test with existing config
- [ ] Test with nvm only
- [ ] Test with fnm only
- [ ] Test with no version managers
- [ ] Test in CI environment (non-TTY)
- [ ] Test all CLI flags

### Task 7.4: Update documentation
- [ ] Update README with init command
- [ ] Document wizard steps with screenshots
- [ ] Document all CLI flags
- [ ] Document configuration options
- [ ] Add troubleshooting section

### Task 7.5: Update CHANGELOG
- [ ] Add entry for v0.8.0
- [ ] List new init command
- [ ] Note setup command is now alias
- [ ] Mention new configuration options
- [ ] Credit contributors

## Phase 8: Polish & Release (Week 3)

### Task 8.1: Code review and refinement
- [ ] Review all code for clarity
- [ ] Optimize hot paths if needed
- [ ] Ensure consistent error handling
- [ ] Check for edge cases
- [ ] Run clippy and fix warnings

### Task 8.2: Update Makefile
- [ ] Add `make wizard-test` for manual wizard testing
- [ ] Update help text with new command
- [ ] Ensure `make dev` works with init command

### Task 8.3: Prepare release
- [ ] Run full test suite
- [ ] Build release binaries
- [ ] Test installation on clean systems
- [ ] Update version to 0.8.0
- [ ] Create git tag

### Task 8.4: Gather feedback
- [ ] Run wizard with beta testers
- [ ] Collect usability feedback
- [ ] Identify confusing prompts
- [ ] Make adjustments based on feedback

## Success Criteria

- [ ] All tasks completed
- [ ] All tests passing
- [ ] Code coverage >85%
- [ ] Documentation updated
- [ ] Wizard completes successfully
- [ ] Generated config works correctly
- [ ] Quick mode and non-interactive mode work
- [ ] User feedback is positive
- [ ] Ready for v0.8.0 release
