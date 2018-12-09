countA :: String -> Int
countA = length . filter (== 'a')

main :: IO ()
main = do
  s <- getLine
  n <- read <$> getLine :: IO Int
  let (d, m) = divMod n (length s)
  let res = d * countA s + countA (take m s)
  print res
