import Data.List (inits)

main :: IO ()
main = do
  [p, d, m, s] <- map read . words <$> getLine
  let res =
        length . last . takeWhile (\i -> s >= sum i) . inits $
        iterate
          (\x ->
             if x - d >= m
               then x - d
               else m)
          p
  print res
