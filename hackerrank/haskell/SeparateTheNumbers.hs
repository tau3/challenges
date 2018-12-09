import Control.Monad

startsWith :: String -> String -> Bool
startsWith _ [] = True
startsWith [] _ = False
startsWith (x:xs) (y:ys) = (x == y) && startsWith xs ys

isBeautiful :: String -> Maybe String
isBeautiful x
  | null solutions = Nothing
  | otherwise = Just $ fst $ head solutions
  where
    solutions =
      filter (\(begin, end) -> splitToNumbers [begin] end) $
      map (`splitAt` x) [1 .. length x `div` 2]
    splitToNumbers _ [] = True
    splitToNumbers [] _ = False
    splitToNumbers xs remains =
      let first = head xs
          next = show ((read first :: Int) + 1)
       in ((remains `startsWith` next) &&
           splitToNumbers (next : xs) (drop (length next) remains))

main :: IO ()
main = do
  q <- read <$> getLine :: IO Int
  replicateM_
    q
    (do s <- getLine
        case isBeautiful s of
          Nothing -> putStrLn "NO"
          Just x -> putStrLn ("YES " ++ show' x)
        return ())
  where
    show' = init . tail . show
