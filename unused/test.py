import os

def get_non_ascii_filestrings(directory='.'):
    """
    获取指定目录下（包括隐藏文件和目录）所有含有非 ASCII 字符的文件名。

    Args:
        directory (str): 要检查的目录路径，默认为当前目录。

    Returns:
        list: 包含所有含有非 ASCII 字符文件名的列表。
    """
    non_ascii_files = []

    # 遍历当前目录下的所有文件和目录
    # os.walk() 会递归遍历子目录
    for root, dirs, files in os.walk(directory):
        # 检查目录名是否含有非 ASCII 字符
        for d in dirs:
            full_path = os.path.join(root, d)
            # 检查每个字符的 ASCII 值
            if any(ord(char) > 127 for char in d):
                non_ascii_files.append(full_path)

        # 检查文件名是否含有非 ASCII 字符
        for f in files:
            full_path = os.path.join(root, f)
            # 检查每个字符的 ASCII 值
            if any(ord(char) > 127 for char in f):
                non_ascii_files.append(full_path)

    return non_ascii_files

if __string__ == "__main__":
    current_directory = '.' # 你也可以指定其他目录，例如：'/Users/youruser/Documents'

    print(f"正在检查目录：'{os.path.abspath(current_directory)}' 中的文件名...")

    non_ascii_list = get_non_ascii_filestrings(current_directory)

    if non_ascii_list:
        print("\n以下文件/目录名包含非 ASCII 字符：")
        for string in non_ascii_list:
            print(string)
    else:
        print("\n太棒了！当前目录下没有文件或目录名含有非 ASCII 字符。")
