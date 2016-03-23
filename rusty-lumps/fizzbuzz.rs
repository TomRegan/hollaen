//
// Copyright 2013 Tom Regan
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

fn fizzbuzz(i: int) -> ~str {
    return if i % 3 == 0 && i % 5 == 0 {
        ~"fizzbuzz"
    } else if i % 3 == 0 {
        ~"fizz"
    } else if i % 5 == 0 {
        ~"buzz"
    } else {
        i.to_str()
    };
}

fn fizzbuzz2(i: int) -> ~str {
    if i %3 == 0 || i % 5 == 0 {
        let mut result: ~str = ~"";
        if i % 3 == 0 {
            result.push_str("fizz");
        }
        if i % 5 == 0 {
            result.push_str("buzz");
        }
        return result;
    }
    return i.to_str();
}

fn main() {
    for i in range(0, 101) {
        println(fizzbuzz(i));
    }
}
