RSpec.describe Wasabi do
  it "has a version number" do
    expect(Wasabi::VERSION).not_to be nil
  end

  describe '.sum' do
    it "1 + 2 = 3" do
      expect(Wasabi.sum(1, 2)).to eq 3
    end
  end

  describe '.call_to_s' do
    context '1.to_s' do
      subject { Wasabi.call_to_s(1) }

      it { is_expected.to eq '1' }
    end

    context 'class with :to_s defined' do
      subject do
        klass = Class.new
        klass.define_method(:to_s) { 'foo' }
        Wasabi.call_to_s(klass.new)
      end

      it { is_expected.to eq 'foo' }
    end
  end
end
