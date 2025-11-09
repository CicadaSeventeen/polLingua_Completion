from pypinyin import pinyin,Style
import itertools

identity = lambda x: x
remove = lambda x: ['']

#返回全拼（小写）, 不使用多音字功能
def string_pinyin_normal(string):
    return ''.join([l[0] for l in pinyin(string,style=Style.NORMAL)])

#返回全拼（小写）, 使用多音字功能
#这个product功能叫做笛卡尔积
def string_pinyin_normal_heteronym(string):
    double_list = pinyin(string,style=Style.NORMAL,heteronym=True)
    return [''.join(comb) for comb in itertools.product(*double_list)]

#返回全拼（小写）, 最大的多音字功能
def string_pinyin_normal_heteronym_all(string):
    double_list = pinyin(list(string),style=Style.NORMAL,heteronym=True)
    return [''.join(comb) for comb in itertools.product(*double_list)]

def string_pinyin_normal(string,string_solver_chinese=identity,string_solver_other=identity):
    double_list_1 = pinyin(string,style=Style.NORMAL)
    double_list_2 = pinyin(string,style=Style.NORMAL,errors="replace")
    double_list = []
    for count in range(0,len(double_list_1)):
        if double_list_1[count] == double_list_2[count]:
            #中文
            s=string_solver_chinese(double_list_1[count])
        else:
            #非中文
            s=string_solver_other(double_list_1[count])
        double_list.append(s)


