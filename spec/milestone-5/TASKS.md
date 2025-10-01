# Milestone 5: Testing & Polish - Tasks

**Timeline:** Weeks 9-10
**Version:** v0.6.0-v0.9.0
**Status:** Not Started

---

## Tasks

### M5.1: Comprehensive unit test suite
- [ ] Config parsing edge cases
- [ ] Version file discovery edge cases
- [ ] Plugin loading edge cases
- [ ] Error handling for all error types
- [ ] Achieve >85% coverage

### M5.2: Integration test suite
- [ ] End-to-end activation scenarios
- [ ] Multi-plugin fallback logic
- [ ] Config override precedence
- [ ] Auto-install flow variations

### M5.3: Shell test suite
- [ ] Hook installation tests (bash, zsh)
- [ ] Directory change detection tests
- [ ] Command evaluation tests
- [ ] Idempotency tests
- [ ] Profile modification tests

### M5.4: Performance benchmarking
- [ ] File discovery benchmark (<5ms target)
- [ ] Plugin matching benchmark (<20ms target)
- [ ] Total activation benchmark (<85ms P95 target)
- [ ] Set up criterion benchmarks

### M5.5: Documentation
- [ ] README.md (quick start, features, comparison)
- [ ] CONTRIBUTING.md (development setup, guidelines)
- [ ] API.md (plugin interface, config schema)
- [ ] Inline code documentation (rustdoc)

### M5.6: Code quality
- [ ] Run clippy (Rust linter)
- [ ] Format with rustfmt
- [ ] Address all warnings
- [ ] Security audit (cargo audit)

---

## Success Criteria

- ✅ All tests passing on CI
- ✅ Coverage >85% (measured by tarpaulin)
- ✅ Benchmarks meet targets (<85ms P95)
- ✅ Documentation complete and reviewed
- ✅ Zero known critical bugs

---

**See [PLAN.md](./PLAN.md) for detailed implementation specifications.**
