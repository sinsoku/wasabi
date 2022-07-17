require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rake/extensiontask"

Rake::ExtensionTask.new("wasabi") do |ext|
  ext.lib_dir = "lib/wasabi"
end

task :default => [:clobber, :compile, :spec]
