#!/bin/bash
cargo run -- \
  -vvvvv \
  --device-id a202e6776b3846e3978a3269af503cc1 \
  --device-password ccff1a3f1aba49dda0c9380e19d3d7233ce75230ab2347de916f31ac23b124f8 \
  ws://localhost:8080/ws
