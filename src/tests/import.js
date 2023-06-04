import "boo";
import foo from './foo';
import {foo as boo} from './boo'
import {foo} from './boo'
import boo, { foo, boo, doo } from "../boo";
import boo, { foo, kung as foo, doo } from "../boo";
import boo, * as foo from "../boo";
import * as foo from "../boo";