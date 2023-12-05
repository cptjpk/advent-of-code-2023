#!/bin/bash

# Define the number of days
NUM_DAYS=25

# Loop through each day starting from day 2
for day in $(seq -f "%02g" 2 $NUM_DAYS)
do
    # Create directory for the day
    DAY_DIR="day$day"
    echo "Creating directory: $DAY_DIR"
    mkdir $DAY_DIR

    # Navigate into the directory
    cd $DAY_DIR

    # Initialize Cargo project
    echo "Initializing Cargo project in $DAY_DIR"
    cargo init

    # Navigate back to root directory
    cd ..
done

# Add all new directories to Git staging
echo "Adding all new directories to Git"
git add day*

# Commit the changes to Git
echo "Committing all new directories to Git"
git commit -m "Initialize Cargo projects for days 2 to $NUM_DAYS"

echo "All days initialized and changes committed."
