require "bundler/gem_tasks"
require "rspec/core/rake_task"
require_relative 'ext/build'

RSpec::Core::RakeTask.new(:spec)

require "rake/extensiontask"

task :build => 'thermite:build'

Rake::ExtensionTask.new("wasabi") do |ext|
  ext.lib_dir = "lib/wasabi"
end

task :default => [:clobber, 'thermite:build', :spec]
