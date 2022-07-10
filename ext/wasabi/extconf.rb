# frozen_string_literal: true

require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("wasabi/wasabi") do |r|
  r.ext_dir = '../..'
end
