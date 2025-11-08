import sys
# p = .24
# k = .324
# b_min = .0074
# b_max = .0123
# circ = 1000000000
# stake_weight = 25247924
# stake_amount = 124288444

p = (int(sys.argv[1]) / 10**18)
k = (int(sys.argv[2]) / 10**18)
b_min = (int(sys.argv[3]) / 10**18)
b_max = (int(sys.argv[4]) / 10**18)
circ = int(sys.argv[5]) / 10**18
stake_weight = int(sys.argv[6]) / 10**18
stake_amount = int(sys.argv[7]) / 10**18

rewards = (circ / 30) * (stake_weight / stake_amount)**p *         \
            (
                (b_max**(1/p) - b_min**(1/p))/k * min(k, stake_amount/circ) + b_min**(1/p)
            )**p

from eth_abi import encode

def abi_encode_uint256(number):
    return encode(['uint256'], [number])

# print(rewards)
# print(str(int(rewards*10**18))+ '0',end='')
# print("\n")
print('0x'+abi_encode_uint256(int(rewards * 10**18)).hex(),end='')
# print("hey",end='')