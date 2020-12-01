(ns advent-of-code.d01
  (:require [clojure.core.logic.fd :as fd]
            [clojure.core.logic :refer :all]
            [advent-of-code.util :as u]
            [clojure.string :as str]))

(defn load-inputs []
  (mapv u/parse-long (str/split-lines (u/get-input 1))))

(defn solve-part1 []
  (let [domain (apply fd/domain (load-inputs))
        [[a b]] (run 1 [a b]
                 (fd/in a domain)
                 (fd/in b domain)
                 (fd/+ a b 2020))]
    (* a b)))


(defn solve-part2 []
  (let [domain (apply fd/domain (load-inputs))
        [[a b c]] (run 1 [a b c]
                       (fresh [s]
                         (fd/in a domain)
                         (fd/in b domain)
                         (fd/in c domain)
                         (fd/+ a b s)
                         (fd/+ s c 2020)))]
    (* a b c)))