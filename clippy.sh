#!/usr/bin/env bash

cargo clippy --no-deps --color always --\
  -W clippy::pedantic\
		-A clippy::must_use_candidate\
		-A clippy::cast_precision_loss\
		-A clippy::cast_possible_truncation\
		-A clippy::cast_possible_wrap\
		-A clippy::cast_sign_loss\
		-A clippy::mut_mut
