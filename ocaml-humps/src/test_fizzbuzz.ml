open OUnit

let test_fizzbuzz_1 _ =
  assert_equal "1" (Fizzbuzz.fizzbuzz 1)

let test_fizzbuzz_3 _ =
  assert_equal "fizz" (Fizzbuzz.fizzbuzz 3)

let test_fizzbuzz_5 _ =
  assert_equal "buzz" (Fizzbuzz.fizzbuzz 5)

let test_fizzbuzz_15 _ =
  assert_equal "fizzbuzz" (Fizzbuzz.fizzbuzz 15)

let test_fizzbuzz_98 _ =
  assert_equal "98" (Fizzbuzz.fizzbuzz 98)

let test_fizzbuzz_100 _ =
  assert_equal "buzz" (Fizzbuzz.fizzbuzz 100)

let suite =
  "Fizzbuzz Tests" >::: [
      "test_fizzbuzz_1"   >:: test_fizzbuzz_1;
      "test_fizzbuzz_3"   >:: test_fizzbuzz_3;
      "test_fizzbuzz_5"   >:: test_fizzbuzz_5;
      "test_fizzbuzz_15"  >:: test_fizzbuzz_15;
      "test_fizzbuzz_100" >:: test_fizzbuzz_100
    ]

let _ =
  run_test_tt_main suite
