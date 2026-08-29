import { useState, useCallback } from "react";

export function useFormState<T extends object>(initialState: T) {
  const [form, setForm] = useState<T>(initialState);

  const updateField = useCallback(<K extends keyof T>(key: K, value: T[K]) => {
    setForm((prev) => ({ ...prev, [key]: value }));
  }, []);

  const resetForm = useCallback(() => {
    setForm(initialState);
  }, [initialState]);

  return { form, setForm, updateField, resetForm };
}
