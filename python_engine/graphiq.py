import functools
import inspect
import time
import json
import sys
import hashlib
from datetime import datetime
from typing import Any, Callable

class giq:
    @staticmethod
    def _get_code_hash(func: Callable) -> str:
        """Extracts DNA using Source -> Bytecode -> Name fallback."""
        hasher = hashlib.blake2b()
        
        # 1. Try to get the literal Source Code (The best way)
        try:
            source = inspect.getsource(func)
            hasher.update(source.encode())
            return hasher.hexdigest()
        except (OSError, TypeError):
            pass 

        # 2. Fallback to Bytecode (If source is missing/hidden)
        # This captures the logic even if the file is gone
        try:
            if hasattr(func, "__code__"):
                hasher.update(func.__code__.co_code) # The actual logic instructions
                hasher.update(str(func.__code__.co_consts).encode()) # The constants used
                return hasher.hexdigest()
        except Exception:
            pass

        # 3. Last Resort: Function Name + Module
        # Better than "unknown", at least we know who it is
        identity = f"{func.__module__}.{func.__name__}"
        return hashlib.blake3(identity.encode()).hexdigest()

    @staticmethod
    def trace(func: Callable):
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            node_name = func.__name__
            code_hash = giq._get_code_hash(func)
            
            start_time = time.time()
            
            try:
                result = func(*args, **kwargs)
                status = "success"
            except Exception as e:
                result = str(e)
                status = "failed"
            
            duration = int((time.time() - start_time) * 1000)
            
            trace_packet = {
                "id": f"{node_name}_{int(time.time())}",
                "label": node_name,
                "code_hash": code_hash,
                "input_hashes": {
                    "args": [str(a) for a in args], 
                    "kwargs": {k: str(v) for k, v in kwargs.items()}
                },
                "output_hashes": {"result_summary": str(result)[:100]}, 
                "duration_ms": duration,
                "status": status
            }
            
            print(f"GRAPHIQ_TRACE:{json.dumps(trace_packet)}")
            sys.stdout.flush() 
            
            return result
        return wrapper