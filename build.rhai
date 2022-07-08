print("executing postscript");

if variable::get("sublibrary") == " " {
    print("Deleting lib folder");
    file::delete("lib");
}


print("done");