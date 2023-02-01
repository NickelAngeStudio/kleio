####################################################
# Extract X.h variables and convert them into Rust #
# Param1 : Path of X.h					
# Param2: Path of destination
#
# Usage : ./extract_xh.sh {path_to_X.h} {dest.rs}	
# NickelAnge.Studio
####################################################


input="$1"
######################################
# $IFS removed to allow the trimming # 
#####################################
while read -r line
do
  #echo "$line"
  
  id=${line:0:2}

  # Get 2 first characters of line as ID 
  #id=$(awk '{print substr($line, 0, 2)}')

  #echo "IFD=[$id]"
  
  # Parse file according to first 2 characters of line
  case $id in

	# Global variable definition start with #d
     "#d")
	  # Extract variable name
	  #varname=${line#*" "}
	  #varname=${line#*" "}
	  #value=${varname#*"	"}
	  #pos1=$(( ${#varname} - ${#value} ))
	  #pos2 = $(( ${#varname} - ${"	"} ))
#	  vn = $(( ${#line} - ${#rest} - ${#searchstring} ))
	  #pos1=$(awk -v a="$line" -v b=" " 'BEGIN{print index(a,b)}')
	  #pos2=$(awk -v a="$line" -v b="	" 'BEGIN{print index(a,b)}')
	  
	  #varname=${line:${pos1}:${pos2}}
	  
	  #varname=$("$line" | sed -n "s/^.*-\s*\(\S*\).*$/\1/p")
	  
	  #varname=$($line| grep -E -P '[:alpha:]+[:alnum:]')
	  
	  #echo $line | sed 's|[a-e]\+|_|g'
      
      varname=$(echo $line | awk '{print $2}')
      value=$(echo $line | awk '{print $3}')
      
      
      
      echo "$varname=$value"
      ;;
      
    # Comments start with /*
    "/*")
      echo "$line"
      ;;
      
    # Event follow up
    " *" | "**" | "* ")
      echo "$line"
      ;;

    
	# Ignore rest
    *)
	  #echo -n "UK=$id\n"
      ;;
  esac

  
  #echo 'someletters_12345_moreleters.ext' | cut -d'_' -f 2
  
  
done < "$input"
