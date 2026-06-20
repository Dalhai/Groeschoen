#!/usr/bin/env ruby
# frozen_string_literal: true

issue = ARGV[0]

if ARGV.length != 1 || issue.nil? || issue.strip.empty? || issue !~ /\A\d+\z/
  warn "Usage: #{File.basename($PROGRAM_NAME)} <issue-number>"
  exit 1
end

exec(
  "claude",
  "-w",
  "issue#{issue}",
  "--permission-mode",
  "auto",
  "Let's work on issue #{issue}.\n" \
  "1. Create a fresh branch off of a newly fetched origin/dev.\n" \
  "2. Make fine grained commits along the way.\n" \
  "3. Create a pull request at the end.\n" \
  "4. Finally, move the ticket to 'In Review'.\n" \
  "Have fun!"
)
