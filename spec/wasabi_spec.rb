RSpec.describe Wasabi do
  it "has a version number" do
    expect(Wasabi::VERSION).not_to be nil
  end

  describe '.sum' do
    it "1 + 2 = 3" do
      expect(Wasabi.sum(1, 2)).to eq 3
    end
  end
end
