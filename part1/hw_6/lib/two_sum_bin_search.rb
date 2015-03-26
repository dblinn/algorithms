class TwoSumBinSearch
  attr_accessor :sorted, :range, :range_bottom, :range_top, :sums_in_range

  def initialize(numbers, range)
    @sorted = numbers.sort
    @lowest = @sorted.first
    @range = range
    @range_bottom = range.begin
    @range_top = range.end
    @sums_in_range = Hash.new(0)

    # calculate_distinct_pairs
    accumulate_all_pairs
    # @sums_in_range.select { |value, count| puts "value=#{value}, count=#{count}" }
  end

  def calculate_distinct_pairs
    accumulate_all_pairs
    @sums_in_range = @sums_in_range.select { |value, count| count == 1 }
  end

  def accumulate_all_pairs
    # one_percent = @sorted.size / 100
    # i = 0
    @sorted.each do |n|
      accumulate_target_pairs(n)

      # i += 1
      # puts "At #{i}" if i % one_percent == 0
    end
  end

  def accumulate_target_pairs(number)
    target = range_bottom - number
    return unless target > number || @lowest > range_bottom

    i = bin_search_index(target)
    return unless i

    sum = @sorted[i] + number
    while sum <= @range_top do
      @sums_in_range[sum] += 1 unless @sorted[i] == number
      # puts "#{number} + #{@sorted[i]} = #{sum}"

      i += 1
      break if i >= @sorted.size
      sum = @sorted[i] + number
    end
  end

  def bin_search_index(target)
    (0...@sorted.size).bsearch { |i| @sorted[i] >= target }
  end
end