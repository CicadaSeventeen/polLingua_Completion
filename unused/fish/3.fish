set want "3.6.0"
set have $FISH_VERSION

if test (printf "%s\n%s\n" $3.5.0 $FISH_VERSION | sort -V | tail -1) = $FISH_VERSION
    echo "$have >= $want"
else
    echo "$have <  $want"
end
