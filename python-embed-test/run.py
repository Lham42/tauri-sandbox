import sys
import io

def execute_code(code):
    # Create a StringIO object to capture print output
    output = io.StringIO()
    
    # Create a dictionary of local variables that will be available to the executed code
    local_vars = {
        'print': lambda *args, **kwargs: print(*args, **kwargs, file=output)
    }
    
    try:
        # Execute the code
        exec(code, local_vars)
        
        # Get the printed output
        printed_output = output.getvalue()
        
        # If there's a 'result' variable, return it along with any printed output
        if 'result' in local_vars:
            return f"Result: {local_vars['result']}\n\nPrinted Output:\n{printed_output}"
        else:
            return f"Printed Output:\n{printed_output}"
    except Exception as e:
        return f"Error: {str(e)}"

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python execute_code.py '<python_code>'")
        sys.exit(1)
    
    # Get the code directly from the command-line argument
    code = sys.argv[1]
    
    # Execute the code and get the result
    print("Executing code...")
    result = execute_code(code)
    
    # # Write the result to result.txt
    # with open('result.txt', 'w') as f:
    #     f.write(result)
    
    # Also print the result to stdout
    print(result)
    
    print("Execution completed. Check result.txt for output.")