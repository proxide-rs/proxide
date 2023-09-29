#!/bin/sh -l

results=$(echo "$@" | xargs proxide)
echo "results=$results" >> $GITHUB_OUTPUT
