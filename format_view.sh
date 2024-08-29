#!/bin/bash

# Script to apply an additional formatting for 
# 'view!' macros from Leptos. Set into the vscode
# rust analyser params : 
# "rust-analyzer.rustfmt.overrideCommand": ["./format_view.sh"]

input=$(cat)

formatted=$(echo "$input" | rustfmt --edition 2021 --emit stdout)

if echo "$formatted" | grep -q 'view!'; then
    echo "$formatted" | leptosfmt --stdin
else
    echo "$formatted"
fi