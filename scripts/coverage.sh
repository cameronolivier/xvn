#!/bin/bash
cargo tarpaulin --out Html --out Lcov --output-dir coverage --exclude-files 'tests/*'
