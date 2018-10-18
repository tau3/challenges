import Data.Ix (inRange)

main :: IO ()
main = do
  let readListOfInt = map read . words <$> getLine :: IO [Int]
  [s, t] <- readListOfInt
  [a, b] <- readListOfInt
  _ <- readListOfInt
  ms <- readListOfInt
  ns <- readListOfInt
  let apples = map (+ a) ms
  let oranges = map (+ b) ns
  print $ length $ filter (inRange (s, t)) apples
  print $ length $ filter (inRange (s, t)) oranges
