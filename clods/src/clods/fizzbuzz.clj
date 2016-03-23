;;; clods.fizzbuzz

;; Copyright 2015 Tom Regan
;;
;; Licensed under the Apache License, Version 2.0 (the "License");
;; you may not use this file except in compliance with the License.
;; You may obtain a copy of the License at
;;
;;     http://www.apache.org/licenses/LICENSE-2.0
;;
;; Unless required by applicable law or agreed to in writing, software
;; distributed under the License is distributed on an "AS IS" BASIS,
;; WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
;; See the License for the specific language governing permissions and
;; limitations under the License.

;;; Commentary:

;; Does fizzbuzz.

;;; Code:

(ns clods.fizzbuzz)

(defn fizzbuzz
  []
  (doseq
      [n (range 1 101)]
    (if (= (mod n 15) 0)
      (println "fizzbuzz")
      (if (= 0 (mod n 5))
        (println "buzz")
        (if (= 0 (mod n 3))
          (println "fizz")
          (println n))))))

;;; clods.fizzbuzz ends here
