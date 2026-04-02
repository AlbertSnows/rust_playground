import os

root = os.environ.get('ZED_WORKTREE_ROOT', os.getcwd())
print(f"root: {root}")

filter_path = os.path.join(root, 'target/debug/.test_filter')
print(f"filter_path: {filter_path}")

filter = open(filter_path).read().strip()
print(f"filter: {filter}")

cmd = 'settings set target.run-args ' + filter + ' --test-threads 1'
print(f"lldb command would be: {cmd}")
