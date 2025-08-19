#include <iostream>
#include <cstddef>
#include <utility>
#include <vector>
#include <string>

union CosyValue {
    int intValue;
    double complexValue[2];
};
enum CosyType {
    NUMBER,
    STRING,
    COMPLEX,
    VECTOR
};
class Cosy {
    public:
        Cosy ( );
        Cosy ( int );
        Cosy ( const std::string& );
        Cosy ( double, double );
        Cosy ( std::vector<Cosy> vec );

        // Conversions
        Cosy into_complex();

        // Operators
        Cosy operator+ ( const Cosy & other ) const;
        friend std::ostream&  operator<< ( std::ostream& os, const Cosy & obj );
    private:
        CosyValue value;
        std::string stringValue;
        std::vector<Cosy> vectorValue;
        CosyType type;
};
Cosy::Cosy ( ) {
    // Default to an integer
    value.intValue = 0;
    type = NUMBER;
};
Cosy::Cosy ( std::vector<Cosy> vec ) {
    vectorValue = std::move(vec);
    type = VECTOR;
};
Cosy::Cosy ( int val ) {
    value.intValue = val;
    type = NUMBER;
};
Cosy::Cosy ( const std::string& str ) {
    stringValue = str;
    type = STRING;
};
Cosy::Cosy ( double real, double imag ) {
    value.complexValue[0] = real;
    value.complexValue[1] = imag;
    type = COMPLEX;
};
Cosy Cosy::into_complex() {
    if ( this->type == NUMBER ) {
        this->type = COMPLEX;
        this->value.complexValue[0] = static_cast<double>(value.intValue);
        this->value.complexValue[1] = 0.0;
    } else if ( this->type == STRING ) {
        throw std::runtime_error("Cannot convert string to complex number!");
    } else if ( this->type == VECTOR ) {
        // Check that there are at least two elements
        if (this->vectorValue.size() < 2) {
            throw std::runtime_error("Cannot convert vector to complex, must have at least two elements!");
        }

        this->type = COMPLEX;
        this->value.complexValue[0] = vectorValue[0].into_complex().value.complexValue[0];
        this->value.complexValue[1] = vectorValue[1].into_complex().value.complexValue[0];
    }
    
    return *this;
}
Cosy Cosy::operator+ ( const Cosy & other ) const {
    Cosy result;
    if ( type == STRING && other.type == STRING ) {
        result.stringValue = stringValue + other.stringValue;
        result.type = STRING;
    } else if ( type == STRING || other.type == STRING ) {
        // Convert non-string to string and concatenate
        std::string left_str, right_str;
        if ( type == STRING ) {
            left_str = stringValue;
        } else if ( type == NUMBER ) {
            left_str = std::to_string(value.intValue);
        } else {
            throw std::runtime_error("Cannot convert complex/vector to string for concatenation!");
        }
        
        if ( other.type == STRING ) {
            right_str = other.stringValue;
        } else if ( other.type == NUMBER ) {
            right_str = std::to_string(other.value.intValue);
        } else {
            throw std::runtime_error("Cannot convert complex/vector to string for concatenation!");
        }
        
        result.stringValue = left_str + right_str;
        result.type = STRING;
    } else if ( type == NUMBER && other.type == COMPLEX ) {
        result.value.complexValue[0] = value.intValue + other.value.complexValue[0];
        result.value.complexValue[1] = other.value.complexValue[1];
        result.type = COMPLEX;
    } else if ( type == COMPLEX && other.type == NUMBER ) {
        result = *this + Cosy(other.value.intValue);
    } else if ( type == NUMBER && other.type == NUMBER ) {
        result.value.intValue = value.intValue + other.value.intValue;
        result.type = NUMBER;
    } else if ( type == COMPLEX && other.type == COMPLEX ) {
        result.value.complexValue[0] = value.complexValue[0] + other.value.complexValue[0];
        result.value.complexValue[1] = value.complexValue[1] + other.value.complexValue[1];
        result.type = COMPLEX;
    }

    return result;
}
std::ostream & operator<< ( std::ostream & os, const Cosy & obj ){
    switch (obj.type) {
        case NUMBER:
            os << obj.value.intValue;
            break;
        case STRING:
            os << obj.stringValue;
            break;
        case COMPLEX:
            if ( obj.value.complexValue[1] >= 0 ) {
                os << "(" << obj.value.complexValue[0] << " + " << obj.value.complexValue[1] << "i)";
            } else {
                os << "(" << obj.value.complexValue[0] << " - " << -obj.value.complexValue[1] << "i)";
            }
            break;
        case VECTOR:
            os << "{ ";
            for ( int i = 0; i < obj.vectorValue.size(); ++i ) {
                os << obj.vectorValue[i];
                if ( i < obj.vectorValue.size() - 1 ) {
                    os << ", ";
                }
            }
            os << " }";
            break;
    }
    return os;
};