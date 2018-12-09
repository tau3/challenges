import Data.HashMap.Strict as M ((!), empty, insert, member)

enumerate :: [a] -> [(Int, a)]
enumerate = zip [0 ..]

solve :: [Int] -> Int
solve = fst . solve'
  where
    solve' xs = foldl step (-1, M.empty) (enumerate xs)
    step (result, cache) (i, x)
      | x `M.member` cache =
        let diff = i - cache ! x
            result' =
              if result < 0
                then diff
                else min result diff
         in (result', cache')
      | otherwise = (result, cache')
      where
        cache' = M.insert x i cache

main :: IO ()
main = do
  _ <- getLine
  a <- map read . words <$> getLine :: IO [Int]
  print $ solve a
