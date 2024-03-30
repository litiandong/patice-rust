#!/usr/bin/bash
SOURCE=""
OUTPUT=""

# 解析命令行参数
while getopts ":s:m:" opt; do
	case $opt in
		s) SOURCE="$OPTARG"
			;;
		m) OUTPUT="$OPTARG"
			;;
		\?) echo "Invalid option -$OPTARG" >&2
			exit 1
		  ;;
	    esac
done
	
# 检查SOURCE文件是否存在
if [ ! -f "$SOURCE" ]; then
	  echo "Source file does not exist."
	    exit 1
fi

# 检查是否提供了必要的参数
if [ -z "$SOURCE" ] || [ -z "$OUTPUT" ]; then
	  echo "Usage: $0 -s [source file] -m [output file]"
	    exit 1
fi

echo "source file: $SOURCE"
echo "output file: $OUTPUT"

# 将source file中的内容用"```rust,no_run"包裹追加到output file中
{
	echo "\`\`\`rust,no_run"
  	cat "$SOURCE"
	echo "\`\`\`"
} >> $OUTPUT



# 清除source file
{
	echo "fn main() {"
	echo "	println!(\"Hello World\");"
	echo "}"
}> $SOURCE

