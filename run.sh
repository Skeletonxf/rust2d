#!/bin/bash
if cargo build --release ; then
  love .
else
  echo "Build failed"
fi
