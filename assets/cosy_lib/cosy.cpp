#include <iostream>
#include <cstddef>
#include <utility>

union CosyValue {
    int intValue;
    double complexValue[2];
};
enum CosyType {
    NUMBER,
    COMPLEX
};
class Cosy {
    public:
        Cosy ( );
        Cosy ( int );
        Cosy ( double, double );

        // Operators
        Cosy operator+ ( const Cosy & other ) const;
        friend std::ostream&  operator<< ( std::ostream& os, const Cosy & obj );
    private:
        CosyValue value;
        CosyType type;
};
Cosy::Cosy ( ) {
    // Default to an integer
    value.intValue = 0;
    type = NUMBER;
};
Cosy::Cosy ( int val ) {
    value.intValue = val;
    type = NUMBER;
};
Cosy::Cosy ( double real, double imag ) {
    value.complexValue[0] = real;
    value.complexValue[1] = imag;
    type = COMPLEX;
};
Cosy Cosy::operator+ ( const Cosy & other ) const {
    Cosy result;
    if ( type == NUMBER && other.type == COMPLEX ) {
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
        case COMPLEX:
            if ( obj.value.complexValue[1] >= 0 ) {
                os << "(" << obj.value.complexValue[0] << " + " << obj.value.complexValue[1] << "i)";
            } else {
                os << "(" << obj.value.complexValue[0] << " - " << -obj.value.complexValue[1] << "i)";
            }
            break;
    }
    return os;
};