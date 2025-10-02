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
- [ ] Implement `prompt_shell()` in `init/prompts.rs`
- [ ] Show detected shell with confirmation (Confirm prompt)
- [ ] If declined, show Select prompt with available shells
- [ ] Display shell profile path
- [ ] Return selected Shell type
- [ ] Add tests with simulated input

### Task 3.2: Implement plugin selection prompt
- [ ] Implement `prompt_plugins()` in `init/prompts.rs`
- [ ] Use MultiSelect with detected managers pre-selected
- [ ] Show manager name and installation path
- [ ] After selection, prompt for priority order
- [ ] Return Vec<String> of plugin names in priority order
- [ ] Handle case where no managers detected (show warning)
- [ ] Add tests with various detection scenarios

### Task 3.3: Implement auto-install mode prompt
- [ ] Implement `prompt_auto_install()` in `init/prompts.rs`
- [ ] Use Select with 3 options: Prompt, Always, Never
- [ ] Add educational descriptions for each option
- [ ] Default to "Prompt" (safest option)
- [ ] Return AutoInstallMode enum
- [ ] Add tests for all options

### Task 3.4: Implement version files prompt
- [ ] Implement `prompt_version_files()` in `init/prompts.rs`
- [ ] Use MultiSelect with .nvmrc, .node-version, .tool-versions
- [ ] Pre-select .nvmrc and .node-version (defaults)
- [ ] After selection, prompt for priority order
- [ ] Return Vec<String> of filenames in priority order
- [ ] Add tests for different selections

### Task 3.5: Implement configuration review prompt
- [ ] Implement `prompt_confirm_config()` in `init/prompts.rs`
- [ ] Display formatted summary of all configuration
- [ ] Use colored output for readability
- [ ] Show config file path and shell profile path
- [ ] Use Confirm prompt for final approval
- [ ] Return bool (confirmed or not)
- [ ] Add tests with various configs

## Phase 4: Wizard Orchestration (Week 2)

### Task 4.1: Implement wizard state management
- [ ] Define `WizardState` struct in `init/wizard.rs`
- [ ] Fields: shell, plugins, auto_install, version_files
- [ ] Implement `Default` for sensible defaults
- [ ] Implement conversion to `Config` struct
- [ ] Add tests for state transitions

### Task 4.2: Implement interactive wizard flow
- [ ] Implement `run_wizard()` in `init/wizard.rs`
- [ ] Step 1: Call `prompt_shell()`, update state
- [ ] Step 2: Call `prompt_plugins()`, update state
- [ ] Step 3: Call `prompt_auto_install()`, update state
- [ ] Step 4: Call `prompt_version_files()`, update state
- [ ] Step 5: Call `prompt_confirm_config()`, confirm state
- [ ] Return WizardState or error
- [ ] Add progress indicators (Step X/5)
- [ ] Add tests for full flow

### Task 4.3: Implement quick mode
- [ ] Implement `run_quick_setup()` in `init/wizard.rs`
- [ ] Auto-detect shell
- [ ] Auto-detect version managers
- [ ] Use defaults: auto_install=prompt, version_files=[.nvmrc, .node-version]
- [ ] Skip all prompts
- [ ] Log decisions to stdout
- [ ] Return WizardState
- [ ] Add tests for quick mode

### Task 4.4: Implement non-interactive mode
- [ ] Implement `run_non_interactive()` in `init/wizard.rs`
- [ ] Similar to quick mode but with logging
- [ ] Use CLI flags if provided (--shell, etc.)
- [ ] Fall back to detection and defaults
- [ ] Return WizardState or error
- [ ] Add tests for non-interactive mode

## Phase 5: Configuration & Installation (Week 2)

### Task 5.1: Implement configuration validation
- [ ] Implement `validate_config()` in `init/validation.rs`
- [ ] Validate shell is supported (bash or zsh)
- [ ] Validate at least one plugin selected
- [ ] Validate auto_install mode is valid
- [ ] Validate version_files not empty
- [ ] Return Result with validation errors
- [ ] Add tests for valid and invalid configs

### Task 5.2: Implement config file generation
- [ ] Implement `generate_config()` in `init/wizard.rs`
- [ ] Convert WizardState to Config struct
- [ ] Add header comments with timestamp and instructions
- [ ] Serialize to YAML format
- [ ] Return formatted config string
- [ ] Add tests for various configurations

### Task 5.3: Implement config file writing
- [ ] Implement `write_config()` in `init/wizard.rs`
- [ ] Check if config exists, handle --force flag
- [ ] If exists and not force, ask to overwrite
- [ ] Preserve custom comments if modifying existing config
- [ ] Write to ~/.xvnrc with proper permissions
- [ ] Return path to written config
- [ ] Add tests for new and existing configs

### Task 5.4: Integrate with existing SetupInstaller
- [ ] Reuse `setup::SetupInstaller` for shell integration
- [ ] Call `installer.install()` after config written
- [ ] Pass shell type from wizard state
- [ ] Capture installation result
- [ ] Add tests for integration

## Phase 6: User Feedback & Output (Week 2)

### Task 6.1: Implement wizard header and branding
- [ ] Implement `print_wizard_header()` in `init/wizard.rs`
- [ ] Use `output::print_header()` for branding
- [ ] Add welcome message
- [ ] Explain what wizard will do
- [ ] Add visual separator

### Task 6.2: Implement success message
- [ ] Implement `print_success_message()` in `init/wizard.rs`
- [ ] Use colored output module
- [ ] Show config file location
- [ ] Show shell profile modified
- [ ] Show next steps (restart shell, test)
- [ ] Mention re-running wizard to modify

### Task 6.3: Implement educational help text
- [ ] Add help text for each wizard step
- [ ] Explain what each option does
- [ ] Provide examples where helpful
- [ ] Add links to documentation
- [ ] Keep text concise but informative

### Task 6.4: Implement error messages
- [ ] Implement error handling for each step
- [ ] Use `output::error()` for errors
- [ ] Provide actionable next steps
- [ ] Handle user cancellation gracefully (Ctrl+C)
- [ ] Log errors appropriately

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
