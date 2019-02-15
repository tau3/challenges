import qualified Data.Set as S

solve :: [Int] -> [Int] -> [Int]
solve scores alice = solve' (S.fromList scores) alice []
  where
    solve' _ [] ps = reverse ps
    solve' sc (a:as) ps = solve' sc' as ps'
      where
        sc' = a `S.insert` sc
        p = S.size sc' - S.findIndex a sc'
        ps' = p : ps

main :: IO ()
main = do
  _ <- getLine
  scores <- map read . words <$> getLine
  _ <- getLine
  alice <- map read . words <$> getLine
  let res = solve scores alice
  mapM_ print res
