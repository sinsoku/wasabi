require 'thermite/tasks'

Thermite::Config.prepend(Module.new do
  # Avoid problems with Requiring .so files
  #
  # ref: https://github.com/rubygems/rubygems/issues/1507
  def shared_library
    @shared_library ||= "#{library_name}.#{RbConfig::CONFIG['DLEXT'] || 'so'}"
  end

  # Pick up a patch to support custom ruby extension directory
  #
  # ref: https://github.com/malept/thermite/pull/49
  def ruby_extension_dir
    @ruby_extension_dir ||= @options.fetch(:ruby_extension_dir, 'lib')
  end

  def ruby_extension_path
    ruby_path(ruby_extension_dir, shared_library)
  end
end)

root_path = File.expand_path('..', __dir__)
Thermite::Tasks.new(
  cargo_project_path: root_path,
  ruby_project_path: root_path,
  ruby_extension_dir: 'lib/wasabi'
)
