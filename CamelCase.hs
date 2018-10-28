solveNonEmpty :: String -> Int
solveNonEmpty = (+ 1) . length . filter (\l -> l `elem` ['A' .. 'Z'])

solve :: String -> Int
solve [] = 0
solve xs = solveNonEmpty xs

main :: IO ()
main = interact $ show . solve
