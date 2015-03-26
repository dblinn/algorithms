require_relative 'spec_helper'
require_relative '../lib/two_sum_bin_search'
require_relative 'test_fixtures'

describe '#two_sum_bin_search' do
  it 'gets the correct answer for a small example' do
    tsbs = TwoSumBinSearch.new([1,3,5,7,11, -9, -4], 0..2)
    expect(tsbs.sums_in_range.values.size).to eq 2
  end

  it 'weeds out duplicates' do
    tsbs = TwoSumBinSearch.new([1,1,3,5,7,11, -6,-9, -4], 0..2)
    expect(tsbs.sums_in_range.values.size).to eq 2
  end

  it 'gets another example correct' do
    tsbs = TwoSumBinSearch.new([1,1,2,3,4,6,8], -10000..10000)
    expect(tsbs.sums_in_range.values.size).to eq 11
  end

  it 'gets the correct answer for a small example that is slightly larger' do
    tsbs = TwoSumBinSearch.new(TestFixtures.small_example, -10000..10000)
    expect(tsbs.sums_in_range.values.size).to eq 6
  end

  it 'accumulates pairs' do
    tsbs = TwoSumBinSearch.new(TestFixtures.million_number_problem, -10000..10000)
    puts tsbs.sums_in_range.values.size
  end
end