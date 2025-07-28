name: Check Rusty Import on Windows

on:
  push:
  pull_request:
  workflow_dispatch:

jobs:
  test-import:
    runs-on: windows-latest
    steps:
      - name: Set up Python
        uses: actions/setup-python@v5
        with:
          python-version: '3.11'

      - name: Install rusty-manga-image-translator
        run: |
          python -m pip install --upgrade pip
          pip install ^
            --extra-index-url https://frederik-uni.github.io/manga-image-translator-rust/python/wheels/simple/ ^
            rusty-manga-image-translator
        shell: cmd

      - name: Test import (should not panic)
        run: python -c "import rusty_manga_image_translator"
        shell: cmd
