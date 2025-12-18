import Control.Monad (replicateM)

readInts :: IO [Int]
readInts = map read . words <$> getLine

solve :: Int -> Int -> Int -> [(Int, Int)] -> Int
solve rq cq n obstacles = sum solutionToEachDirection
  where
    solutionToEachDirection = map (\x -> countDirection x 0 (rq, cq)) nexts
    nexts =
      [ \(r, c) -> (r - 1, c),
        \(r, c) -> (r - 1, c + 1),
        \(r, c) -> (r, c + 1),
        \(r, c) -> (r + 1, c + 1),
        \(r, c) -> (r + 1, c),
        \(r, c) -> (r + 1, c - 1),
        \(r, c) -> (r, c - 1),
        \(r, c) -> (r - 1, c - 1)
      ]
    countDirection :: ((Int, Int) -> (Int, Int)) -> Int -> (Int, Int) -> Int
    countDirection calcNext current pos
      | inBounds pos =
          if pos `elem` obstacles
            then current - 1
            else countDirection calcNext (current + 1) (calcNext pos)
      | otherwise = current - 1
    inBounds :: (Int, Int) -> Bool
    inBounds (r, c) = (r >= 0) && (c >= 0) && (r < n) && (c < n)

solveRaw :: [String] -> Int
solveRaw (x : y : z) =
  let [n, k] = map read $ words x :: [Int]
      [rq, cq] = map read $ words y :: [Int]
      obstacles = map (map read . words) z :: [[Int]]
   in solve (rq - 1) (cq - 1) n (map listToTuple obstacles)

main :: IO ()
main = do
  [n, k] <- readInts
  [rq, cq] <- readInts
  obstacles <- replicateM k $ listToTuple <$> readInts
  print $ solve (rq - 1) (cq - 1) n obstacles

listToTuple :: [Int] -> (Int, Int)
listToTuple [a, b] = (a - 1, b - 1)
listToTuple xs = error $ "not a tuple: " ++ show xs
