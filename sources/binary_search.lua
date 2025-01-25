--[[
   table.binsearch( table, value [, compval [, reversed] ] )
   
   Searches the table through BinarySearch for the given value.
   If the  value is found:
      it returns a table holding all the mathing indices (e.g. { startindice,endindice } )
      endindice may be the same as startindice if only one matching indice was found
   If compval is given:
      then it must be a function that takes one value and returns a second value2,
      to be compared with the input value, e.g.:
      compvalue = function( value ) return value[1] end
   If reversed is set to true:
      then the search assumes that the table is sorted in reverse order (largest value at position 1)
      note when reversed is given compval must be given as well, it can be nil/_ in this case
   Return value:
      on success: a table holding matching indices (e.g. { startindice,endindice } )
      on failure: nil
]]--
do
    -- Avoid heap allocs for performance
    local default_fcompval = function( value ) return value end
    local fcompf = function( a,b ) return a < b end
    local fcompr = function( a,b ) return a > b end
    function table.binsearch( tbl,value,fcompval,reversed )
       -- Initialise functions
       local fcompval = fcompval or default_fcompval
       local fcomp = reversed and fcompr or fcompf
       --  Initialise numbers
       local iStart,iEnd,iMid = 1,#tbl,0
       -- Binary Search
       while iStart <= iEnd do
          -- calculate middle
          iMid = math.floor( (iStart+iEnd)/2 )
          -- get compare value
          local value2 = fcompval( tbl[iMid] )
          -- get all values that match
          if value == value2 then
             local tfound,num = { iMid,iMid },iMid - 1
             while value == fcompval( tbl[num] ) do -- ERROR: this may cause fail in fcompval if num is out of range and tbl[num] is nil
                tfound[1],num = num,num - 1
             end
             num = iMid + 1
             while value == fcompval( tbl[num] ) do -- ERROR: this may cause fail in fcompval if num is out of range and tbl[num] is nil
                tfound[2],num = num,num + 1
             end
             return tfound
          -- keep searching
          elseif fcomp( value,value2 ) then
             iEnd = iMid - 1
          else
             iStart = iMid + 1
          end
       end
    end
 end
 -- CHILLCODE™