#!/bin/bash

find . -type f -name '*.rs' | entr ./run.bash
