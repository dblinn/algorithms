require_relative './spec_helper'
require_relative './test_fixtures'
require_relative '../lib/two_sum'

describe 'two_sum' do
  describe '#distinct_pair_for' do
    it 'finds distinct pairs' do
      expect(TwoSum.new([-1,1]).distinct_pair_for(0)).to be_truthy
    end

    it 'does not count a pair if there is more than one of a kind of number' do
      expect(TwoSum.new([-1,1,1]).distinct_pair_for(0)).to be_falsey
    end

    it 'reports when there is not a distinct pair for a target' do
      expect(TwoSum.new([-1,2,3,4]).distinct_pair_for(0)).to be_falsey
    end

    it 'operates on a large data set' do
      expect(TwoSum.new(TestFixtures.million_number_problem).distinct_pair_for(100)).to be_falsey
    end
  end

  describe '#targets_with_distinct_two_sums' do
    it 'finds the number of distinct pairs for targets in an array' do
      expect(TwoSum.new([-1,2,3,4]).targets_with_distinct_two_sums([0,1,3,4,5])).to eq [1,3,5]
    end

    it 'only includes targets with distinct pairs' do
      expect(TwoSum.new([-2,-1,2,3,4]).targets_with_distinct_two_sums([1,3,4,5])).to eq [3,5]
    end
  end

  context 'it finds the answer' do
    it 'finds the answer' do
      puts TwoSum.new(TestFixtures.million_number_problem)
            .targets_with_distinct_two_sums((-10000..10000).to_a).size
    end
  end
end