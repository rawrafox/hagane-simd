$: << "#{__dir__}/generator"

require "fileutils"
require "shellwords"

require "simd-bridge-generator"

task default: [:simd_bridge]

task simd_bridge: ["#{__dir__}/src/.simd"]

rule "#{__dir__}/src/.simd" => "#{__dir__}/generator/simd-bridge-generator.rb" do |t|
  puts "Generating SIMD bridge from #{t.source}"

  Bridge::SIMD.generate("#{__dir__}/src").map do |name, contents|
    FileUtils.mkdir_p(File.dirname(name))

    File.write(name, contents)
  end

  File.write(t.name, "")
end
