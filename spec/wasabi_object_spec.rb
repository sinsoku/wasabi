RSpec.describe Wasabi::Object do
  # TODO: support args
  # let(:obj) { Wasabi::Object.new("foo") }
  let(:obj) { Wasabi::Object.new }

  describe "#name" do
    it "returns the initialization args" do
      expect(obj.name).to eq "foo"
    end
  end

  describe "#say" do
    it "returns the greeting message" do
      expect(obj.say).to eq "say, foo"
    end
  end
end
