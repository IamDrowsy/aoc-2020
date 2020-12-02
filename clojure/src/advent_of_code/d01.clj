(ns advent-of-code.d01
  (:require [clojure.core.logic.fd :as fd]
            [clojure.core.logic :as l]
            [advent-of-code.util :as u]
            [clojure.string :as str]))

(defn load-inputs []
  (mapv u/parse-long (str/split-lines (u/get-input 1))))

(defn solve-part1 []
  (let [[[a b]]
        (l/run 1 [a b]
             (fd/in a b (apply fd/domain (load-inputs)))
             (fd/+ a b 2020))]
    (* a b)))

(defn solve-part2 []
  (let [[[a b c]]
        (l/run 1 [a b c]
             (fd/in a b c (apply fd/domain (load-inputs)))
             (fd/eq (= 2020 (+ a b c))))]
    (* a b c)))