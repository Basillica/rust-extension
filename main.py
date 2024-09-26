import matrix_mul

# result = matrix_mul.sum_as_string(5, 6)
# print(result, type(result))
# print(matrix_mul.module_b.hello_world())


# class Sample(object):
#     def __init__():
#         print("hello form call sample")


# sample = Sample()


#ä###
from matrix_mul import version

print("the module version is: ", version())
print(matrix_mul.sum_as_string(5, 6))

###
from matrix_mul import functions
print(functions.func_null())