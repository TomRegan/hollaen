(defproject hello "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "http://github.com/TomRegan/Clollo"
  :license {:name "Apache 2.0"
            :url "http://www.apache.org/licenses/LICENSE-2.0.txt"}
  :dependencies [[org.clojure/clojure "1.6.0"]
                 [seesaw "1.4.5"]]
  :main ^:skip-aot hello.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all}})
