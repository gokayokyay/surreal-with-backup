ps -elf | head -1; ps -elf | awk '{if ($5 == 1) {print $0}}'
