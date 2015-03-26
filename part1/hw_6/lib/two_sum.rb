class TwoSum
  attr_accessor :number_counts, :distinct, :numbers

  def initialize(numbers)
    @numbers = numbers
    @number_counts = {}
    numbers.each { |number| insert_number(number) }
    @distinct = build_distinct
  end

  def targets_with_distinct_two_sums(targets)
    total = 0
    targets.select do |target|
      found = distinct_pair_for(target)
      if found
        puts "FOUND DISTINCT PAIR FOR #{target}"
        total += 1
      elsif target % 100 == 0
        puts "AT #{target}, Found a total of #{total}"
      end
      found
    end
  end

  def distinct_pair_for(target)
    matches = 0
    @distinct.each do |number, _|
      difference = target - number
      matches += @distinct.has_key?(difference) ? 1 : 0
    end

    matches == 2
  end

  private

  def insert_number(number)
    if @number_counts[number]
      @number_counts[number] += 1
    else
      @number_counts[number] = 1
    end
  end

  def build_distinct
    @number_counts.select { |_, count| count <= 1 }
  end
end