# Milestone 5: Testing & Polish

**Timeline:** Weeks 9-10  
**Status:** Planning  
**Version:** v0.6.0-v0.9.0

---

## Plan

### Goal

Achieve comprehensive test coverage, benchmark performance, and complete documentation.

### Deliverables

- [ ] Comprehensive unit test suite (>85% coverage)
- [ ] Integration tests with mock plugins
- [ ] Shell integration tests (bash, zsh)
- [ ] Error handling edge cases
- [ ] Performance benchmarking
- [ ] Documentation (README, CONTRIBUTING, API)

### Test Categories

1. **Unit Tests:** Config, version files, plugins, errors
2. **Integration Tests:** End-to-end activation with mocks
3. **Shell Tests:** Hook installation, directory detection
4. **Benchmarks:** Activation time, file search, config load

### Performance Targets

- File discovery: <5ms
- Plugin matching: <20ms
- Total activation: <85ms (P95)

### Documentation Requirements

- README.md (quick start, installation, troubleshooting)
- CONTRIBUTING.md (development setup, plugin guide)
- API documentation (rustdoc)

### Success Criteria

- All tests passing on CI
- Coverage >85%
- Benchmarks meet targets
- Documentation complete
- Zero known critical bugs

---

## Architecture

### Testing Strategy

**Unit Tests:**
- Pure functions tested in isolation
- Mock external dependencies
- Property-based tests for invariants

**Integration Tests:**
- Multi-component interactions
- Mock plugin implementations
- Config precedence scenarios

**Shell Tests:**
- Real bash/zsh execution
- FD:3 protocol verification
- Profile modification checks

### Benchmarking

Using `criterion` crate for performance measurement:
- Activation flow end-to-end
- Version file search (nested directories)
- Config loading
- Plugin availability checks

See [ARCHITECTURE.md](../ARCHITECTURE.md#testing-architecture) for comprehensive testing strategy.

---

